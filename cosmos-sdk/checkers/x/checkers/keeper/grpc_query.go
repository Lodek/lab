package keeper

import (
	"github.com/lodek/lab.git/cosmos-sdk/checkers/x/checkers/types"
)

var _ types.QueryServer = Keeper{}
