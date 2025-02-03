use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_305941: FileFormat = FileFormat {
    id: 305_941,
    source_type: SourceType::Wikidata,
    name: "vCard",
    extensions: &["vcard", "vcf"],
    media_types: &["text/vcard"],
    internal_signatures: &[],
    related_formats: &[],
};
