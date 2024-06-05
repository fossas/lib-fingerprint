
Testing images for JAR fingerprinting.

These are saved in git lfs, so all you should need to do is:
```shell
git lfs pull
```

If desired, you can reconstruct with the following:
```shell
rm -f hazelcast_managementcenter_5_3_1.tar
rm -f bitnami_elasticsearch_7_17_17_debian_11_r4.tar
rm -f elasticsearch_7_17_17.tar

docker pull hazelcast/management-center:5.3.1
docker pull elasticsearch:7.17.17
docker pull bitnami/elasticsearch:7.17.7-debian-11-r4

docker save hazelcast/management-center:5.3.1 > hazelcast_managementcenter_5_3_1.tar
docker save bitnami/elasticsearch:7.17.7-debian-11-r4 > bitnami_elasticsearch_7_17_17_debian_11_r4.tar
docker save elasticsearch:7.17.17 > elasticsearch_7_17_17.tar
```
