#!/bin/bash

if [ $# -gt 0 ]
  then
    if [ $1 = "web" ]
      then
        docker exec -it react-rust-simple-scoreboard-web-1 /bin/bash
        exit 1
    elif [ $1 = "api" ]
      then
        docker exec -it react-rust-simple-scoreboard-api-1 /bin/bash
        exit 1
      fi 
fi

echo "Usage: ./shell.sh [web|api]"