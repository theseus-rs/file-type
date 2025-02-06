use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_65595616: FileFormat = FileFormat {
    id: 65_595_616,
    source_type: SourceType::Wikidata,
    name: "Variant Call Format",
    extensions: &["vcf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
