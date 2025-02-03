use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111272315: FileFormat = FileFormat {
    id: 111_272_315,
    source_type: SourceType::Wikidata,
    name: "Ensoniq SQ80 instrument file",
    extensions: &["efs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
