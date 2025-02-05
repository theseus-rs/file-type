use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29904545: FileFormat = FileFormat {
    id: 29_904_545,
    source_type: SourceType::Wikidata,
    name: "Statistical Analysis System data set index",
    extensions: &["sas7bndx", "si2", "si7"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
