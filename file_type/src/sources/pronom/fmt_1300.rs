use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1300: FileFormat = FileFormat {
    id: 2_118,
    puid: "fmt/1300",
    name: "Broderbund The Print Shop/PrintMaster/American Greetings Project",
    extensions: &[
        "ban", "bro", "biz", "cal", "car", "cer", "env", "fax", "sig", "cft", "hcr", "lbl", "let",
        "nws", "not", "pcr", "php", "tsh", "web", "sti",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
