use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27959911: FileFormat = FileFormat {
    id: 27_959_911,
    source_type: SourceType::Wikidata,
    name: "ATRAC Advanced Lossless",
    extensions: &["aa3", "aal", "at3", "oma", "omg"],
    media_types: &["audio/ATRAC-ADVANCED-LOSSLESS"],
    signatures: &[],
    related_formats: &[],
};
