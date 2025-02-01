use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_65595754: FileFormat = FileFormat {
    id: 65_595_754,
    puid: "wikidata/65595754",
    name: "Variant Call Format",
    extensions: &["vcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
