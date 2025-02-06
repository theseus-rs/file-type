use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63280227: FileFormat = FileFormat {
    id: 63_280_227,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Windows, version 4.0a",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
