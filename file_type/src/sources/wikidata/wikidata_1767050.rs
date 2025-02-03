use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1767050: FileFormat = FileFormat {
    id: 1_767_050,
    source_type: SourceType::Wikidata,
    name: "OpenOffice.org XML",
    extensions: &[
        "stc", "std", "sti", "stw", "sxc", "sxd", "sxg", "sxi", "sxm", "sxw",
    ],
    media_types: &[
        "application/vnd.sun.xml.calc",
        "application/vnd.sun.xml.calc.template",
        "application/vnd.sun.xml.draw",
        "application/vnd.sun.xml.draw.template",
        "application/vnd.sun.xml.impress",
        "application/vnd.sun.xml.impress.template",
        "application/vnd.sun.xml.math",
        "application/vnd.sun.xml.writer",
        "application/vnd.sun.xml.writer.global",
        "application/vnd.sun.xml.writer.template",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
