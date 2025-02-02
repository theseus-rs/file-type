use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_87121995: FileFormat = FileFormat {
    id: 87_121_995,
    source_type: SourceType::Wikidata,
    name: "Open Financial Exchange 2.1.1",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx"],
    internal_signatures: &[],
    related_formats: &[],
};
