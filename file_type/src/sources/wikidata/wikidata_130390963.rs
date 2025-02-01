use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130390963: FileFormat = FileFormat {
    id: 130_390_963,
    puid: "wikidata/130390963",
    name: "Objective-C++ source code file",
    extensions: &["mm"],
    media_types: &["text/x-objective-c++"],
    internal_signatures: &[],
    related_formats: &[],
};
