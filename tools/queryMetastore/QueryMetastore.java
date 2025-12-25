import java.sql.*;

public class QueryMetastore {
  public static void main(String[] args) {
    String metastorePath = System.getenv("DERBY_METASTORE_PATH");
    if (metastorePath == null || metastorePath.isEmpty()) {
      metastorePath = "metastore_db"; // Default to relative path
    }
    String url = "jdbc:derby:" + metastorePath;
    try {
      Class.forName("org.apache.derby.jdbc.EmbeddedDriver");
      try (Connection conn = DriverManager.getConnection(url)) {
        System.out.println("Connected to Derby metastore.");

        // List databases (DBS table)
        System.out.println("\nDatabases (DBS):");
        try (Statement s = conn.createStatement();
             ResultSet rs = s.executeQuery("SELECT DB_ID, NAME FROM DBS")) {
          while (rs.next()) {
            System.out.printf("DB_ID=%d NAME=%s\n", rs.getLong("DB_ID"),
                              rs.getString("NAME"));
          }
        }

        // List tables per database
        System.out.println("\nTables (TBLS) by database:");
        try (Statement s = conn.createStatement();
             ResultSet rs =
                 s.executeQuery("SELECT TBL_ID, TBL_NAME, DB_ID FROM TBLS")) {
          while (rs.next()) {
            System.out.printf("TBL_ID=%d TBL_NAME=%s DB_ID=%d\n",
                              rs.getLong("TBL_ID"), rs.getString("TBL_NAME"),
                              rs.getLong("DB_ID"));
          }
        }

        // Show partitions (if any)
        System.out.println("\nPartitions (PARTITIONS) sample:");
        try (Statement s = conn.createStatement();
             ResultSet rs =
                 s.executeQuery("SELECT PART_ID, PART_NAME, SD_ID FROM " +
                                "PARTITIONS FETCH FIRST 20 ROWS ONLY")) {
          while (rs.next()) {
            System.out.printf("PART_ID=%d PART_NAME=%s SD_ID=%d\n",
                              rs.getLong("PART_ID"), rs.getString("PART_NAME"),
                              rs.getLong("SD_ID"));
          }
        } catch (SQLException e) {
          System.out.println(
              "No PARTITIONS table or unable to query partitions: " +
              e.getMessage());
        }
      }
    } catch (Exception e) {
      e.printStackTrace();
      System.exit(2);
    }
  }
}
