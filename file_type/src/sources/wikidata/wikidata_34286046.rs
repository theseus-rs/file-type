use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34286046: FileFormat = FileFormat {
    id: 34_286_046,
    source_type: SourceType::Wikidata,
    name: "Pixie script",
    extensions: &["pxi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
