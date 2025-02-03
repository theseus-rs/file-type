use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_108837047: FileFormat = FileFormat {
    id: 108_837_047,
    source_type: SourceType::Wikidata,
    name: "Nero CD-ROM Boot Compilation",
    extensions: &["nrb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
