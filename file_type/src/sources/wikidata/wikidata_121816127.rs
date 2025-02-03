use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_121816127: FileFormat = FileFormat {
    id: 121_816_127,
    source_type: SourceType::Wikidata,
    name: "vCard 3",
    extensions: &["vcard", "vcf"],
    media_types: &["text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
