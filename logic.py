import time
import math

datapoint_n = 0
def get_current_temperature():
    global datapoint_n
    datapoint_n += 1
    if datapoint_n <= 1000:
        return datapoint_n
    return math.sin(datapoint_n-1000) / (datapoint_n-1000) * 500 + 1000

def find_average(list):
    return sum(list) / len(list)

def find_amp(list):
    return max(list) - min(list)
 


def find_freq(list, base): # returns how many times the graph has reached base
    output = 0
    for i in range(len(list)):
        if list[i] == base and list[i-1] != base:
            output += 1
    return output

MAX_AVERAGE_DIVERGENCE = 0.1
MAX_AMP = 1
MAX_TIME_REACH_TARGET = 10
MAX_TIME_STABILIZE = 10

def set_temp(t):
    #send temperature variable to the sensor

    t /= 10

    time_start = time.time()
    target_temperature_reached = False
    history_size = 10
    historical_data = [0]*history_size

    while True:
        current_t = get_current_temperature() / 10
        if current_t >= t:
            target_temperature_reached = True
            break
        if time.time() - time_start >= MAX_TIME_REACH_TARGET:
            return 0 #send NOT OK code

    print("Temperature reached, now stabilizing")

    time_start = time.time()
    for i in range(history_size):
        current_t = get_current_temperature() / 10 
        historical_data[i] = current_t

    while True:
        current_t = get_current_temperature() / 10
        historical_data.pop(0)
        historical_data.append(current_t)
        amp = find_amp(historical_data)
        avg = find_average(historical_data)
        print(f"avg: {avg};    amp: {amp}")
        if datapoint_n % 47 == 0:
            print(historical_data)

        if amp <= MAX_AMP and abs(avg - t) < MAX_AVERAGE_DIVERGENCE:
            return 1 #send OK code
        
        if time.time() - time_start >= MAX_TIME_STABILIZE:
            return 0 #send NOT OK code


print(set_temp(1000))
