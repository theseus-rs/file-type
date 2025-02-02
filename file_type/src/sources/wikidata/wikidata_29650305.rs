use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650305: FileFormat = FileFormat {
    id: 29_650_305,
    source_type: SourceType::Wikidata,
    name: "PSI-MI XML",
    extensions: &["dag", "def"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
