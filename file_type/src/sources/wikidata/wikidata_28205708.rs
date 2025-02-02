use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205708: FileFormat = FileFormat {
    id: 28_205_708,
    source_type: SourceType::Wikidata,
    name: "Applixware Bitmap",
    extensions: &["im"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
