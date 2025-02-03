use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113324648: FileFormat = FileFormat {
    id: 113_324_648,
    source_type: SourceType::Wikidata,
    name: "Pixlr Layered Image",
    extensions: &["pxd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
