use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28207241: FileFormat = FileFormat {
    id: 28_207_241,
    source_type: SourceType::Wikidata,
    name: "SBIG CCDOPS image",
    extensions: &["sbig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
