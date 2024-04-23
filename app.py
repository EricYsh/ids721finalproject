from transformers import pipeline
import sys
import json

def summarize_text(text, max_length=25):
    try:
        summarizer = pipeline(task="summarization", model="Falconsai/text_summarization")
        summarization = summarizer(text, max_length=max_length, min_length=10, do_sample=False)
        return summarization
    except Exception as e:
        return {"error": str(e)}

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python app.py <input_text> [max_length]")
        sys.exit(1)

    input_text = sys.argv[1]
    max_length = int(sys.argv[2]) if len(sys.argv) > 2 else 25

    summarized_results = summarize_text(input_text, max_length)
    print(json.dumps(summarized_results))
