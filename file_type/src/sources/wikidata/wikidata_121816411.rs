use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121816411: FileFormat = FileFormat {
    id: 121_816_411,
    source_type: SourceType::Wikidata,
    name: "vCard 4",
    extensions: &["vcard", "vcf"],
    media_types: &["text/vcard"],
    signatures: &[],
    related_formats: &[],
};
