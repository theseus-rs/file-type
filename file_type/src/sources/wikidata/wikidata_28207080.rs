use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28207080: FileFormat = FileFormat {
    id: 28_207_080,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Names file",
    extensions: &["sdr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
