use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96271500: FileFormat = FileFormat {
    id: 96_271_500,
    source_type: SourceType::Wikidata,
    name: "FlagMaker file format",
    extensions: &["flag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
