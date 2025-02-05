use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_20191913: FileFormat = FileFormat {
    id: 20_191_913,
    source_type: SourceType::Wikidata,
    name: "Apple Help File Format",
    extensions: &["lproj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
