use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125822813: FileFormat = FileFormat {
    id: 125_822_813,
    source_type: SourceType::Wikidata,
    name: "Microsoft Help Combined Full-text Search file",
    extensions: &["chq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
