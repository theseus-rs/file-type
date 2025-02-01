use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7391883: FileFormat = FileFormat {
    id: 7_391_883,
    puid: "wikidata/7391883",
    name: "SNP file format",
    extensions: &["snp", "snp"],
    media_types: &["application/vnd.ms-access", "image/x-snp"],
    internal_signatures: &[],
    related_formats: &[],
};
