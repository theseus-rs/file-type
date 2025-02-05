use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29167436: FileFormat = FileFormat {
    id: 29_167_436,
    source_type: SourceType::Wikidata,
    name: "Microsoft Object Description Language",
    extensions: &["odl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
