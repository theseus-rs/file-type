use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111182163: FileFormat = FileFormat {
    id: 111_182_163,
    source_type: SourceType::Wikidata,
    name: "Dreamweaver Webpage Template",
    extensions: &["dwt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
