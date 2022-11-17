from brownie import SimpleStorage, accounts

# run this test with brownie test

# brownie test -k test_deploy # will only run the test_deploy function
# brownie test --pdb will automatically open a python shell where the test is failing
# the flags are coming from pytest

def test_deploy():
    # Arrange
    account = accounts[0]
    
    # Act
    simple_storage = SimpleStorage.deploy({"from":account})
    starting_value = simple_storage.retrieve()
    expected = 0

    # Assert
    assert starting_value == expected

def test_updating_storage():
    # Arrange
    account = accounts[0]
    simple_storage = SimpleStorage.deploy({"from":account})
    
    # Act
    expected = 15
    simple_storage.store(expected, {"from":account})
    # Assert
    assert expected == simple_storage.retrieve()