def get_bathroom_visits(guest_data, closing_time):
    """Calculate all bathroom visits for a guest given their drinking and bathroom time."""
    drink_time, bathroom_time = guest_data
    visits = []
    current_time = drink_time  # First visit happens after first drinking session
    
    while current_time <= closing_time:
        # Only add the visit if there's enough time to start using the bathroom
        if current_time < closing_time:
            visits.append((current_time, current_time + bathroom_time))
        current_time += drink_time + bathroom_time
    
    return visits

def count_overlapping_visits(all_visits, time):
    """Count how many bathroom visits are active at a given time."""
    count = 0
    for start, end in all_visits:
        if start <= time < end:
            count += 1
    return count

def find_min_restrooms(n, closing_time, guest_data):
    """Find minimum number of restrooms needed."""
    # If closing time is 0, no restrooms needed
    if closing_time == 0:
        return 0
        
    # Get all bathroom visits for all guests
    all_visits = []
    for guest in guest_data:
        all_visits.extend(get_bathroom_visits(guest, closing_time))
    
    # If no visits, return 0
    if not all_visits:
        return 0
    
    # Create list of all unique times when bathroom usage changes
    critical_times = set()
    for start, end in all_visits:
        critical_times.add(start)
        critical_times.add(end)
    
    # Check maximum overlap at each critical time
    max_overlap = 0
    for time in critical_times:
        overlap = count_overlapping_visits(all_visits, time)
        max_overlap = max(max_overlap, overlap)
    
    return max_overlap

def main():
    # Read input
    n, closing_time = map(int, input().split())
    guest_data = []
    for _ in range(n):
        drink_time, bathroom_time = map(int, input().split())
        guest_data.append((drink_time, bathroom_time))
    
    # Calculate and output result
    result = find_min_restrooms(n, closing_time, guest_data)
    print(result)

if __name__ == "__main__":
    main()