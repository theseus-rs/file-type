use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_119823553: FileFormat = FileFormat {
    id: 119_823_553,
    source_type: SourceType::Wikidata,
    name: "BNTX",
    extensions: &["bntx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
