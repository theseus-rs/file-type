use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130390963: FileFormat = FileFormat {
    id: 130_390_963,
    source_type: SourceType::Wikidata,
    name: "Objective-C++ source code file",
    extensions: &["mm"],
    media_types: &["text/x-objective-c++"],
    signatures: &[],
    related_formats: &[],
};
