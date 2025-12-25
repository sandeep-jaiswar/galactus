#!/usr/bin/env python3
"""
Python Hive Metastore inspector using `hmsclient`.
This script connects to the Hive metastore Thrift service and lists databases and tables.
"""
import sys

HOST = 'localhost'
PORT = 9083


def query_with_hmsclient(host=HOST, port=PORT):
    try:
        from hmsclient import HMSClient
    except Exception:
        raise ImportError('hmsclient not available')
    # construct with keyword args to avoid positional mis-assignment
    client = HMSClient(host=host, port=port)
    # open the thrift connection (no args)
    client.open()
    try:
        dbs = client.get_all_databases()
        print('Databases:', dbs)
        for db in dbs:
            tables = client.get_all_tables(db)
            print(f'Database: {db} Tables: {tables}')
            for t in tables:
                table = client.get_table(db, t)
                location = table.sd.location if table.sd else '<no sd>'
                input_format = table.sd.inputFormat if table.sd else '<no sd>'
                print(f'  {db}.{t} -> location={location} inputFormat={input_format}')
    finally:
        try:
            client.close()
        except Exception as e:
            # Ignore close errors but log them so resource issues are diagnosable.
            print(f'Warning: failed to close HMSClient cleanly: {e}', file=sys.stderr)


if __name__ == '__main__':
    try:
        query_with_hmsclient()
    except ImportError:
        print('hmsclient is not installed. Install with: pip install hmsclient thrift')
        sys.exit(3)
    except Exception as e:
        print('Error querying metastore:', e)
        sys.exit(2)
