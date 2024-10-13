# The locally running program is able to take 4850 requests at once in unoptimized and all 5000 requests in optimized mode
for i in {1..5000}; do
    curl -X POST http://localhost:8000/submit \
         -H "Content-Type: application/x-www-form-urlencoded" \
         -d "content=This+was+from+curl+request" &
done

