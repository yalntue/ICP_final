fn main() {
    // Data structures representing users and bids
struct User {
    address: String,
    balance: u64,
}

struct Bid {
    bidder: String,
    amount: u64,
}

// Smart Contract
struct AuctionContract {
    owner: String,
    items: Vec<Item>,
    bids: Vec<Bid>,
    users: Vec<User>,
}

struct Item {
    name: String,
    description: String,
    current_bid: u64,
    highest_bidder: String,
    active: bool,
}

impl AuctionContract {
    // Constructor for creating a new AuctionContract
    fn new(owner: String) -> Self {
        AuctionContract {
            owner,
            items: Vec::new(),
            bids: Vec::new(),
            users: Vec::new(),
        }
    }

    // Method for listing an item for auction
    fn list_item(&mut self, name: String, description: String) {
        let item = Item {
            name,
            description,
            current_bid: 0,
            highest_bidder: "".to_string(),
            active: true,
        };
        self.items.push(item);
    }

    // Method for placing a bid on an item
    fn bid(&mut self, bidder: String, item_index: usize, amount: u64) {
        let item = &mut self.items[item_index];
        if item.active && amount > item.current_bid {
            // Accept the first bid
            item.current_bid = amount;
            item.highest_bidder = bidder.clone();

            // Refund the previous bidder's balance
            if let Some(prev_bid) = self.bids.iter_mut().find(|b| b.bidder == bidder) {
                let prev_bid_amount = prev_bid.amount;
                prev_bid.amount -= amount;
                // Update the bidder's balance
                if let Some(user) = self.find_user_by_address(&bidder) {
                    user.balance += prev_bid_amount;
                }
            }

            // Record the new bid
            self.bids.push(Bid {
                bidder,
                amount,
            });
        }
    }

    // Method for updating the description of a listed item
    fn update_item(&mut self, owner: String, item_index: usize, new_description: String) {
        if owner == self.owner {
            if let Some(item) = self.items.get_mut(item_index) {
                item.description = new_description;
            }
        }
    }

    // Method for stopping the listing of an item
    fn stop_item(&mut self, owner: String, item_index: usize) {
        if owner == self.owner {
            if let Some(item) = self.items.get_mut(item_index) {
                item.active = false;
                // Set the highest bidder as the owner of the item
                if !item.highest_bidder.is_empty() {
                    item.highest_bidder = item.name.clone();
                }
            }
        }
    }

    // Method for finding a user by their address
    fn find_user_by_address(&mut self, address: &str) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.address == address)
    }
}

}
