use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_89029185: FileFormat = FileFormat {
    id: 89_029_185,
    source_type: SourceType::Wikidata,
    name: "American Greetings Project 10-23",
    extensions: &[
        "ban", "biz", "bro", "cal", "car", "cer", "cft", "env", "fax", "hcr", "lbl", "let", "not",
        "nws", "pcr", "php", "sig", "sti", "tsh", "web",
    ],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
