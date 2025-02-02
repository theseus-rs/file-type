use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111182163: FileFormat = FileFormat {
    id: 111_182_163,
    source_type: SourceType::Wikidata,
    name: "Dreamweaver Webpage Template",
    extensions: &["dwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
