# To download/extract nightly tests, run:
# CONSENSUS_SPECS_TEST_VERSION=nightly make
CONSENSUS_SPECS_TEST_VERSION ?= v1.7.0-alpha.8
# Gnosis-preset reference tests are published by gnosischain/consensus-specs on its own
# release cadence (tag derived from that repo's VERSION.txt). Override as needed; pinned in CI.
GNOSIS_SPECS_TEST_VERSION ?= v1.7.0-alpha.8
REPO_NAME := consensus-spec-tests
OUTPUT_DIR := ./$(REPO_NAME)

BLS_TEST_REPO_NAME := bls12-381-tests
BLS_TEST_VERSION := v0.1.1
BLS_TEST = bls_tests_yaml
BLS_OUTPUT_DIR := $(OUTPUT_DIR)/$(BLS_TEST_REPO_NAME)
BLS_BASE_URL := https://github.com/ethereum/$(BLS_TEST_REPO_NAME)/releases/download/$(BLS_TEST_VERSION)

.PHONY: all clean gnosis

all: clean $(OUTPUT_DIR) $(BLS_OUTPUT_DIR)

# Download/extract only the Gnosis-preset suite (+ preset-independent general tests) into the
# same OUTPUT_DIR layout (tests/gnosis/..., tests/general/...). Used by the dedicated gnosis job.
gnosis: $(OUTPUT_DIR)/tests/gnosis $(BLS_OUTPUT_DIR)

$(OUTPUT_DIR)/tests/gnosis:
	mkdir -p $(OUTPUT_DIR)
	# EF general + minimal: only MainnetEthSpec is aliased to GnosisEthSpec, so the MinimalEthSpec
	# column still runs the real (EF) minimal tests, including minimal-only suites like
	# sync_committee_updates that don't exist in the mainnet-shaped gnosis vectors.
	EF_TESTS_PRESETS="general minimal" ./download_test_vectors.sh $(CONSENSUS_SPECS_TEST_VERSION)
	# Gnosis-preset vectors (the aliased mainnet column reads tests/gnosis).
	EF_TESTS_REPO=gnosischain/consensus-specs EF_TESTS_PRESETS="gnosis" \
		EF_TESTS_NIGHTLY_WORKFLOW=tests.yml EF_TESTS_NIGHTLY_BRANCH=master \
		./download_test_vectors.sh $(GNOSIS_SPECS_TEST_VERSION)
	for test_tarball in general.tar.gz minimal.tar.gz gnosis.tar.gz; do \
		tar -xzf $$test_tarball -C $(OUTPUT_DIR); \
		rm -f $$test_tarball; \
	done

clean:
	rm -rf *.tar.gz $(OUTPUT_DIR) $(BLS_OUTPUT_DIR)

$(OUTPUT_DIR):
	mkdir -p $(OUTPUT_DIR)
	./download_test_vectors.sh $(CONSENSUS_SPECS_TEST_VERSION)
	for test_tarball in *.tar.gz; do \
		tar -xzf $$test_tarball -C $(OUTPUT_DIR); \
		rm -f $$test_tarball; \
	done

$(BLS_OUTPUT_DIR):
	mkdir -p $(BLS_OUTPUT_DIR)
	curl --progress-bar --location --remote-name --show-error --retry 3 --retry-all-errors --fail \
		$(BLS_BASE_URL)/$(BLS_TEST).tar.gz
	tar -xzf *.tar.gz -C $(BLS_OUTPUT_DIR)
	rm -f *.tar.gz
