use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59653785: FileFormat = FileFormat {
    id: 59_653_785,
    source_type: SourceType::Wikidata,
    name: "Maya Binary File Format, 32 bit",
    extensions: &["mb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
