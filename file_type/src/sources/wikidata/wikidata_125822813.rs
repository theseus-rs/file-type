use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125822813: FileFormat = FileFormat {
    id: 125_822_813,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help Combined Full-text Search file",
    extensions: &["chq"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
