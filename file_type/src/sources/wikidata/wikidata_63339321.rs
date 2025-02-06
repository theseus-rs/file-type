use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63339321: FileFormat = FileFormat {
    id: 63_339_321,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 4.5a",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
