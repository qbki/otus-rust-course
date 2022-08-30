echo 'Initial state:'
curl http://localhost:8000
echo ''
echo ''
echo 'Added a new room:';
curl -X POST -F 'name=Some New Room' http://localhost:8000/room
curl http://localhost:8000
echo '';
echo '';
echo 'Added a new thermometer and outlet:';
curl -X POST -F 'name=Some New Thermometer' -F 'device_type=THERMOMETER' 'http://localhost:8000/room/Some%20New%20Room/device'
curl -X POST -F 'name=Some New Outlet' -F 'device_type=OUTLET' 'http://localhost:8000/room/Some%20New%20Room/device'
curl http://localhost:8000
echo '';
echo '';
echo 'Shown a room:';
curl http://localhost:8000/room/Some%20New%20Room
echo '';
echo '';
echo 'Shown a device:';
curl http://localhost:8000/room/Some%20New%20Room/device/Some%20New%20Outlet
echo '';
echo '';
echo 'Removed a device:';
curl -X DELETE http://localhost:8000/room/Some%20New%20Room/device/Some%20New%20Outlet
curl http://localhost:8000
echo '';
echo '';
echo 'Removed a room:';
curl -X DELETE http://localhost:8000/room/Some%20New%20Room
curl http://localhost:8000
echo '';
