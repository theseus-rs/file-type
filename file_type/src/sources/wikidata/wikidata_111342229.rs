use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342229: FileFormat = FileFormat {
    id: 111_342_229,
    source_type: SourceType::Wikidata,
    name: "Sounder sound file",
    extensions: &["sndr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
