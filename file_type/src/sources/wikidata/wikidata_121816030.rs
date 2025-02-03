use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121816030: FileFormat = FileFormat {
    id: 121_816_030,
    source_type: SourceType::Wikidata,
    name: "vCard 2.1",
    extensions: &["vcard", "vcf"],
    media_types: &["text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
