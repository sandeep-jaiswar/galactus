import java.util.List;
import org.apache.hadoop.hive.conf.HiveConf;
import org.apache.hadoop.hive.metastore.HiveMetaStoreClient;
import org.apache.hadoop.hive.metastore.api.Table;

public class ThriftQuery {
  public static void main(String[] args) throws Exception {
    HiveConf conf = new HiveConf();
    conf.setVar(HiveConf.ConfVars.METASTOREURIS, "thrift://localhost:9083");
    HiveMetaStoreClient client = new HiveMetaStoreClient(conf);

    List<String> dbs = client.getAllDatabases();
    System.out.println("Databases: " + dbs);
    for (String db : dbs) {
      List<String> tables = client.getAllTables(db);
      System.out.println("Database: " + db + " Tables: " + tables);
      for (String t : tables) {
        Table table = client.getTable(db, t);
        String location =
            table.getSd() != null ? table.getSd().getLocation() : "<no sd>";
        String inputFormat =
            table.getSd() != null ? table.getSd().getInputFormat() : "<no sd>";
        System.out.println(
            String.format("  %s.%s -> location=%s inputFormat=%s", db, t,
                          location, inputFormat));
      }
    }
    client.close();
  }
}
