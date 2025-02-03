use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205679: FileFormat = FileFormat {
    id: 28_205_679,
    source_type: SourceType::Wikidata,
    name: "Amber ARR Bitmap Image",
    extensions: &["arr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
