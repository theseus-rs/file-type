use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_108837047: FileFormat = FileFormat {
    id: 108_837_047,
    source_type: SourceType::Wikidata,
    name: "Nero CD-ROM Boot Compilation",
    extensions: &["nrb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
