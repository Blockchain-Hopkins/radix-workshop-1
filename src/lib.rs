use scrypto::prelude::*;

blueprint! {
    struct Bounty {
        test_package: PackageAddress,
        bounty_bucket: Bucket,
        claimed: bool,
        claimable: bool
    }

    impl Bounty {
        fn instantiate_bounty(test_package: PackageAddress, bounty_bucket: Bucket) -> ComponentAddress {
            let new_bounty: BountyComponent = Self {
                test_package: test_package,
                bounty_bucket: bounty_bucket,
                claimed: false,
                claimable: true
            }
            .instantiate();

            new_bounty.globalize();
        }
    }

}
