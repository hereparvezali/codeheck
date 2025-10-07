echo "Starting codeheck_db..."
docker start codeheck_db
echo "Starting codeheck_queue..."
docker start codeheck_queue

sleep 4

echo "Starting api_server..."
cargo run &

echo "Starting worker..."
cd ../worker && cargo run &
cd ../api_server
