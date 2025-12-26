# Galactus - NSE Market Intelligence Platform
# Use OpenJDK 11 base so Spark runs with a compatible JDK
FROM eclipse-temurin:11-jdk

# Install Python and other system dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    python3-venv \
    wget \
    curl \
    procps \
    && rm -rf /var/lib/apt/lists/*

RUN ln -s /usr/bin/python3 /usr/bin/python || true
RUN ln -s /usr/bin/pip3 /usr/bin/pip || true

# Set Java environment
ENV JAVA_HOME=/usr/local/openjdk-11
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
ENV PYTHONPATH="${SPARK_HOME}/python:${SPARK_HOME}/python/lib/py4j-*-src.zip:${PYTHONPATH}"

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
RUN pip install --no-cache-dir --break-system-packages -r requirements.txt

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
