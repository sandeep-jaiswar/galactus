# Galactus - NSE Market Intelligence Platform
FROM python:3.10-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    openjdk-17-jdk-headless \
    wget \
    curl \
    procps \
    && rm -rf /var/lib/apt/lists/*

# Set Java environment
ENV JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64
ENV PATH="${JAVA_HOME}/bin:${PATH}"

# Install Spark
ENV SPARK_VERSION=3.4.1
ENV HADOOP_VERSION=3
ENV SPARK_HOME=/opt/spark
RUN wget -q "https://archive.apache.org/dist/spark/spark-${SPARK_VERSION}/spark-${SPARK_VERSION}-bin-hadoop${HADOOP_VERSION}.tgz" \
    && tar xzf "spark-${SPARK_VERSION}-bin-hadoop${HADOOP_VERSION}.tgz" -C /opt \
    && mv "/opt/spark-${SPARK_VERSION}-bin-hadoop${HADOOP_VERSION}" "${SPARK_HOME}" \
    && rm "spark-${SPARK_VERSION}-bin-hadoop${HADOOP_VERSION}.tgz"

ENV PATH="${SPARK_HOME}/bin:${PATH}"
ENV PYTHONPATH="${SPARK_HOME}/python:${SPARK_HOME}/python/lib/py4j-0.10.9.7-src.zip:${PYTHONPATH}"

# Download Hudi bundle
ENV HUDI_VERSION=0.15.0
ENV SCALA_VERSION=2.12
RUN wget -q "https://repo1.maven.org/maven2/org/apache/hudi/hudi-spark3.4-bundle_${SCALA_VERSION}/${HUDI_VERSION}/hudi-spark3.4-bundle_${SCALA_VERSION}-${HUDI_VERSION}.jar" \
    -O "${SPARK_HOME}/jars/hudi-spark3.4-bundle_${SCALA_VERSION}-${HUDI_VERSION}.jar"

# Create app directory
WORKDIR /app

# Copy requirements first for better caching
COPY requirements.txt .

# Install Python dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY . .

# Create necessary directories
RUN mkdir -p /app/data/bronze /app/data/silver /app/data/gold /app/logs /tmp/galactus

# Set environment variables
ENV GALACTUS_PROJECT_ROOT=/app
ENV GALACTUS_DATA_ROOT=/app/data
ENV GALACTUS_LOG_DIR=/app/logs
ENV PYTHONPATH=/app:${PYTHONPATH}
ENV SPARK_MASTER=local[*]

# Expose ports (if needed for services)
EXPOSE 8080 4040

# Default command
CMD ["/bin/bash"]
