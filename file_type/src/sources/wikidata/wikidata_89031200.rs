use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_89031200: FileFormat = FileFormat {
    id: 89_031_200,
    source_type: SourceType::Wikidata,
    name: "The Print Shop Project 2-5",
    extensions: &["psproj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
