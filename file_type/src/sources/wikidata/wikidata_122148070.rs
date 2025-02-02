use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122148070: FileFormat = FileFormat {
    id: 122_148_070,
    source_type: SourceType::Wikidata,
    name: "Finale Performance Assessment",
    extensions: &["fpa"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
