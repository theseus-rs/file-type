use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205541: FileFormat = FileFormat {
    id: 28_205_541,
    source_type: SourceType::Wikidata,
    name: "NeoDesk icon",
    extensions: &["nic"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
