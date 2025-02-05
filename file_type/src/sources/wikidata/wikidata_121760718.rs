use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_121760718: FileFormat = FileFormat {
    id: 121_760_718,
    source_type: SourceType::Wikidata,
    name: "Adobe Acrobat MIME Encoded Job Definition File",
    extensions: &["mjd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
