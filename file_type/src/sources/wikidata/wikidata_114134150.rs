use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114134150: FileFormat = FileFormat {
    id: 114_134_150,
    source_type: SourceType::Wikidata,
    name: "MOPAC format",
    extensions: &["mop"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
